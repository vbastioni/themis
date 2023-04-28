// To parse this data:
//
//   import { Convert, Settings } from "./file";
//
//   const settings = Convert.toSettings(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export type Attributes = keyof Pick<
    Settings,
    "displayedAttributes"
    | "searchableAttributes"
    | "filterableAttributes"
    | "sortableAttributes"
>;

export const allAttributes = [
    "id",
    "ancien_id",
    "origine",
    "url",
    "nature",
    "titre_txt",
    "document_bureautique",
    "numero",
    "siret",
    "code_unite_signataire",
    "date_maj",
    "date_depot",
    "date_texte",
    "date_effet",
    "date_fin",
    "date_diffusion",
    "code_ape",
    "code_idcc",
    "raison_sociale",
    "conforme_version_integrale",
    "secteur",
    "themes",
];

export interface Settings {
    displayedAttributes: string[];
    searchableAttributes: string[];
    filterableAttributes: string[];
    sortableAttributes: string[];
    rankingRules: string[];
    stopWords: string[];
    synonyms: Synonyms;
    distinctAttribute: null;
    typoTolerance: TypoTolerance;
    faceting: Faceting;
    pagination: Pagination;
}

export interface Faceting {
    maxValuesPerFacet: number;
}

export interface Pagination {
    maxTotalHits: number;
}

export interface Synonyms {
}

export interface TypoTolerance {
    enabled: boolean;
    minWordSizeForTypos: MinWordSizeForTypos;
    disableOnWords: any[];
    disableOnAttributes: any[];
}

export interface MinWordSizeForTypos {
    oneTypo: number;
    twoTypos: number;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toSettings(json: string): Settings {
        return cast(JSON.parse(json), r("Settings"));
    }

    public static settingsToJson(value: Settings): string {
        return JSON.stringify(uncast(value, r("Settings")), null, 2);
    }
}

function invalidValue(typ: any, val: any, key: any, parent: any = ''): never {
    const prettyTyp = prettyTypeName(typ);
    const parentText = parent ? ` on ${parent}` : '';
    const keyText = key ? ` for key "${key}"` : '';
    throw Error(`Invalid value${keyText}${parentText}. Expected ${prettyTyp} but got ${JSON.stringify(val)}`);
}

function prettyTypeName(typ: any): string {
    if (Array.isArray(typ)) {
        if (typ.length === 2 && typ[0] === undefined) {
            return `an optional ${prettyTypeName(typ[1])}`;
        } else {
            return `one of [${typ.map(a => { return prettyTypeName(a); }).join(", ")}]`;
        }
    } else if (typeof typ === "object" && typ.literal !== undefined) {
        return typ.literal;
    } else {
        return typeof typ;
    }
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = '', parent: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key, parent);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) { }
        }
        return invalidValue(typs, val, key, parent);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases.map(a => { return l(a); }), val, key, parent);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue(l("array"), val, key, parent);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue(l("Date"), val, key, parent);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue(l(ref || "object"), val, key, parent);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, key, ref);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key, ref);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val, key, parent);
    }
    if (typ === false) return invalidValue(typ, val, key, parent);
    let ref: any = undefined;
    while (typeof typ === "object" && typ.ref !== undefined) {
        ref = typ.ref;
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems") ? transformArray(typ.arrayItems, val)
                : typ.hasOwnProperty("props") ? transformObject(getProps(typ), typ.additional, val)
                    : invalidValue(typ, val, key, parent);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function l(typ: any) {
    return { literal: typ };
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "Settings": o([
        { json: "displayedAttributes", js: "displayedAttributes", typ: a("") },
        { json: "searchableAttributes", js: "searchableAttributes", typ: a("") },
        { json: "filterableAttributes", js: "filterableAttributes", typ: a("") },
        { json: "sortableAttributes", js: "sortableAttributes", typ: a("") },
        { json: "rankingRules", js: "rankingRules", typ: a("") },
        { json: "stopWords", js: "stopWords", typ: a("") },
        { json: "synonyms", js: "synonyms", typ: r("Synonyms") },
        { json: "distinctAttribute", js: "distinctAttribute", typ: null },
        { json: "typoTolerance", js: "typoTolerance", typ: r("TypoTolerance") },
        { json: "faceting", js: "faceting", typ: r("Faceting") },
        { json: "pagination", js: "pagination", typ: r("Pagination") },
    ], false),
    "Faceting": o([
        { json: "maxValuesPerFacet", js: "maxValuesPerFacet", typ: 0 },
    ], false),
    "Pagination": o([
        { json: "maxTotalHits", js: "maxTotalHits", typ: 0 },
    ], false),
    "Synonyms": o([
    ], false),
    "TypoTolerance": o([
        { json: "enabled", js: "enabled", typ: true },
        { json: "minWordSizeForTypos", js: "minWordSizeForTypos", typ: r("MinWordSizeForTypos") },
        { json: "disableOnWords", js: "disableOnWords", typ: a("any") },
        { json: "disableOnAttributes", js: "disableOnAttributes", typ: a("any") },
    ], false),
    "MinWordSizeForTypos": o([
        { json: "oneTypo", js: "oneTypo", typ: 0 },
        { json: "twoTypos", js: "twoTypos", typ: 0 },
    ], false),
};

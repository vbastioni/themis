import "tailwindcss/tailwind.css";

import { ChangeEvent, PropsWithChildren, ReactNode, useState } from "react";
import { QueryFunctionContext, useQuery } from "react-query";
import Card from "@components/card";
import { formatSiret } from "@components/helpers/formatters";
import Tag from "@components/tag";

function setState<T>(state: T, setState: React.Dispatch<React.SetStateAction<T>>) {
    return function (key: keyof T) {
        return function (value: ChangeEvent<HTMLInputElement>) {
            setState({ ...state, [key]: value.target.value });
        }
    }
}

interface SiderProps {
    filters: Record<string, any>;
    addFilter?: (key: string, value: any) => void;
    removeFilter?: (key: string, value: { value: any } | { id: number }) => void;
}

function extractArray(record: Record<string, any>, key: string): string[] {
    let sirets: string[];
    const raw = record[key];
    if (raw === undefined) {
        sirets = [];
    } else if (Array.isArray(raw)) {
        sirets = raw;
    } else {
        sirets = [raw];
    }
    return sirets;
}

function Sider(props: SiderProps) {
    const { filters, addFilter, removeFilter } = props;
    const [state, lState] = useState<{
        entreprise: string,
        keyword: string,
    }>({ entreprise: "", keyword: "" })
    const rSetState = setState(state, lState);
    const { entreprise, keyword } = state;

    const sirets = extractArray(filters, "siret");
    const raison_sociales = extractArray(filters, "raison_sociale");

    return (
        <div className="flex-initial w-64 h-[100%] flex flex-col">
            <div>Ici, on cherche</div>
            <br />
            <div>Entreprise :</div>
            <div>
                {raison_sociales.map((rs, i) => (
                    <Tag
                        key={`tag-siret-${i}`}
                        tagText={`Raison sociale : ${rs}`}
                        onClick={() => removeFilter?.("raison_sociale", { id: i })}
                    />
                ))}
                {sirets.map((siret, i) => (
                    <Tag
                        key={`tag-siret-${i}`}
                        tagText={`SIRET: ${formatSiret(siret)}`}
                        onClick={() => removeFilter?.("siret", { id: i })}
                    />
                ))}
            </div>
            <label >
                <input
                    value={entreprise}
                    onChange={rSetState("entreprise")}
                    placeholder="Raison sociale / SIRET"
                />
            </label>
            <div>Mot clefs :</div>
            <label>
                <input
                    value={keyword}
                    onChange={rSetState("keyword")}
                    placeholder="Mot(s) Clef(s)"
                />
            </label>
        </div>
    );
}

function Header() {
    return (
        <div>
            Header
        </div>
    );
}

function Main({ children, header }: PropsWithChildren<{ header?: ReactNode }>) {
    return (
        <div className="flex-auto w-64">
            {header}
            <div>
                {children}
            </div>
        </div>
    )
}

interface LayoutProps {
    header?: ReactNode,
    sider?: ReactNode,
}

function Layout(props: PropsWithChildren<LayoutProps>) {
    const { children, header, sider, } = props;

    return (
        <div className="app">
            <div className="bg-gray-100">
                <div className="container mx-auto py-8">
                    <div className="flex" style={{ width: "100%" }}>
                        {sider}
                        <Main header={header}>
                            {children}
                        </Main>
                    </div>
                </div>
            </div>
        </div>
    );
}

async function fetchDocuments({ queryKey: [_, filters] }: QueryFunctionContext<[string, Record<string, any>], any>) {
    const url = new URL("http://localhost:7700/indexes/acco/search");
    Object.keys(filters).forEach((key) => {
        const value = filters[key];
        if (Array.isArray(value)) {
            if (value.length === 0) {
                return;
            }
            url.searchParams.append("filter", `${key} IN [${value.map((s) => `"${s}"`).join(", ")}]`);
        } else {
            url.searchParams.append("filter", `${key} = ${value}`);
        }
    });
    const method = "GET";
    return await fetch(url.href, { method })
        .then(async (r) => {
            if (r.ok) {
                return await r.json() as MeiliSearch<AccoHit>;
            }
        });
}

function Home() {
    const [filters, setFilters] = useState<Record<string, any>>({});
    const { data, status } = useQuery(['documents', filters], fetchDocuments);
    const addFilter = (key: string, value: any) =>
        setFilters({ ...filters, [key]: [...(filters[key] ?? []), value] });
    const removeFilter = (key: string, value: { value: any } | { id: number }) => {
        if ("id" in value) {
            const id = value.id;
            const newFilters = [...filters[key]];
            newFilters.splice(id);
            setFilters({
                ...filters,
                [key]: newFilters,
            });
        }
    }
    return (
        <Layout
            header={<Header />}
            sider={<Sider filters={filters} removeFilter={removeFilter} />}
        >
            <button onClick={() => setFilters({ ...filters, counter: (filters.counter ?? 0) + 1 })}>
                incr
            </button>
            <div>
                {status === "success" && data?.hits.map((hit, i) => (
                    <Card
                        key={`acco-${i}-${hit.numero}`} data={hit}
                        addFilter={addFilter}
                    />
                ))}
            </div>
        </Layout>
    );
}

export default Home;

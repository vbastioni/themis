interface Theme {
    code: string;
    libelle: string;
    groupe: string;
}

interface AccoHit {
    titre_txt: string;
    numero: string;
    id: string;
    siret: string;
    date_maj: number;
    date_depot: number;
    date_texte: number;
    date_effet: number;
    date_fin: number;
    date_diffusion: number;
    code_ape: string;
    code_idcc: string;
    raison_sociale: string;
    secteur: string;
    themes: Theme[];
}

interface MeiliSearch<T> {
    estimatedTotalHits: number;
    limit: number;
    offset: number;
    processingTimeMs: number;
    query: string;
    hits: T[];
}

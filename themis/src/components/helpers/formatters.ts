export function epochToDate(epoch: number): Date {
    const d = new Date(0);
    d.setUTCSeconds(epoch);
    return d;
}

export function formatSiret(siret: string): string {
    switch (siret.length) {
        case 9: return siret;
        case 14: return `${siret.substring(0, 8)} ${siret.substring(9)}`;
        case 15: return siret[9] === " " ? siret : "";
        default: return "";
    }
}

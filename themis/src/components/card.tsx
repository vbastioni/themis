import { Dispatch, SetStateAction } from "react";
import { epochToDate, formatSiret } from "./helpers/formatters";

interface CardProps {
    data: AccoHit;
    addFilter?: (key: string, value: any) => void;
}

/*
date_effet
date_diffusion
code_ape
code_idcc
secteur
themes
**/

function Card(props: CardProps) {
    const {
        data: { titre_txt, raison_sociale, siret, date_effet, date_diffusion, },
        addFilter,
    } = props;
    return (
        <div className="block p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 mb-10">
            <h6 className="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                {titre_txt}
            </h6>
            <p
                className="font-normal text-gray-700 dark:text-gray-400"
                onClick={() => addFilter?.("raison_sociale", raison_sociale)}
            >
                {`Raison sociale : ${raison_sociale}`}
            </p>
            <p
                className="font-normal text-gray-700 dark:text-gray-400"
                onClick={() => addFilter?.("siret", siret)}
            >
                {`SIRET : ${formatSiret(siret)}`}
            </p>
            <p className="font-normal text-gray-700 dark:text-gray-400">
                {`Date d'effet : ${epochToDate(date_effet).toLocaleDateString()}`}
            </p>
            <p className="font-normal text-gray-700 dark:text-gray-400">
                {`Date diffusion : ${epochToDate(date_diffusion).toLocaleDateString()}`}
            </p>
        </div>
    )
}

export default Card;

import "tailwindcss/tailwind.css";

import { useState } from 'react';
import { Params, useLoaderData, useMatch } from 'react-router-dom';

import AttributesSelector from '@components/attributes-selector';
import { Attributes, Settings, allAttributes } from '../../../../models/settings';

export const Catch = function () {
    const { params: { indice } } = useMatch("/admin/settings/:indice")!;
    return (<div>Index `{indice}` invalide.</div>);
}

export const Loader = async ({ params: { indice } }: { params: Params }) =>
    await fetch(`http://localhost:7700/indexes/${indice}/settings`)
        .then(async (r) => r.ok
            ? await r.json()
            : new Error("invalid indice")
        );

function IndiceSettings() {
    const data: Settings = useLoaderData() as any;
    const [settings, setSettings] = useState(data);
    const updateAttribute = (key: Attributes) => (list: string[]) => 
        setSettings({ ...settings, [key]: list });

    return (
        <div className="app">
            <div className="bg-gray-100">
                <div className="container mx-auto py-8">
                    <h1 className="text-2xl font-bold mb-6 text-center"><i>Acco</i> Indice Settings</h1>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="displayed">Displayed Attributes</label>
                        <AttributesSelector
                            setList={updateAttribute('displayedAttributes')}
                            refList={allAttributes}
                            list={settings.displayedAttributes}
                        />
                    </div>
                    <h1 className="text-2xl font-bold mb-6 text-center"><i>Acco</i> Indice Settings</h1>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="displayed">Filterable Attributes</label>
                        <AttributesSelector
                            setList={updateAttribute('filterableAttributes')}
                            refList={allAttributes}
                            list={settings.filterableAttributes}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
}


export default IndiceSettings;

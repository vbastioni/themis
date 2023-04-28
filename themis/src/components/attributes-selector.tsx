import Tag from "./tag";

interface AttributesProps {
    list: string[],
    setList: (lst: string[]) => void,
    refList: string[],
}

function AttributesSelector(props: AttributesProps) {
    const { list, setList, refList } = props;

    const selected = refList.map((attr) =>
        ({ attr, isSelected: list.some((e) => e === attr) }));
    const sum = selected.reduce((acc, cur) => acc + Number(cur.isSelected), 0);
    return (
        <div>
            <div>
                # Attributs séléctionnés: {sum}
            </div>
            <div className='grid gap-4 grid-flow-row-dense grid-cols-5'>
                {selected.map(({ attr, isSelected }, id) => (
                    <Tag
                        key={`tag-${id}`}
                        style={{ cursor: "pointer", userSelect: "none" }}
                        selected={isSelected}
                        tagText={attr}
                        onClick={() => {
                            const nl = [...list];
                            if (isSelected) {
                                const index = list.indexOf(attr);
                                (index !== -1) && nl.splice(index, 1);
                            } else {
                                nl.push(attr);
                            }
                            setList(nl);
                        }}
                    />
                ))}
            </div>
        </div>
    );
}

export default AttributesSelector;

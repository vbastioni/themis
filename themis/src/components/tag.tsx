interface TagProps {
    selected?: boolean,
    tagText: string,
    onClick?: () => void,
    style?: React.CSSProperties,
}

function Tag(props: TagProps) {
    const { selected, tagText, style, onClick } = props;
    const [bg, text] = selected
        ? ["bg-green-200", "text-green-700"]
        : ["bg-orange-200", "text-orange-700"];
    return (
        <div
            className={`text-xs inline-flex items-center font-bold leading-sm uppercase px-3 py-1 ${bg} ${text} rounded-full`}
            onClick={onClick}
            style={style}
        >
            {tagText}
        </div>
    )
}

Tag.defaultProps = {
    selected: true,
}

export default Tag;

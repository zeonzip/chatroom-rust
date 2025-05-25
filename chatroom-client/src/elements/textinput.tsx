import {useState} from "react";

const defFunc: () => void = () => {};

type MessageInputProps = {
    onEnter?: (content: string) => void,
    onChangeE?: (e: React.ChangeEvent<HTMLInputElement>) => void,
    placeholder?: string,
    clearContentOnEnter?: boolean,
}

function TextInput({ onEnter = defFunc, onChangeE = defFunc,  placeholder = "", clearContentOnEnter = false }: MessageInputProps) {
    const [content, setContent] = useState("");

    function onChange(e: React.ChangeEvent<HTMLInputElement>) {
        setContent(e.target.value);
        onChangeE(e);
    }

    function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key == "Enter" && content != "") {
            const tc = content;
            if (clearContentOnEnter) { setContent(""); }
            onEnter(tc);
        }
    }

    return <input className="border" placeholder={placeholder} type="text" onChange={onChange} onKeyDown={onKeyDown} value={content}/>
}

export default TextInput
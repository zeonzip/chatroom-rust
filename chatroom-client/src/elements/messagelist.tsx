import {Children} from "react";

type MessageListProps = {
    messages: string[],
}

function Messagelist({ messages }: MessageListProps) {
    return <div>
        <ul className="flex flex-col justify-start items-start overflow-y-auto">
            {Children.map(messages, (child, i) => {
                return <li key={i}>{child}</li>;
            })}
        </ul>
    </div>
}

export default Messagelist;
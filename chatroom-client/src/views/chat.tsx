import MessageList from "../elements/messagelist.tsx";
import TextInput from "../elements/textinput.tsx";
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";

type ChatViewProps = {
    onConnectionEnded: (res: any) => void;
}

function ChatView({ onConnectionEnded }: ChatViewProps) {
    const [messages, setMessages] = useState<string[]>([]);

    function pushMessage(msg: string) {
        setMessages(prev => [...prev, msg]);
    }

    function sendMessage(msg: string) {
        invoke('send', {message: msg}).then(_r => {

        }).catch((_r) => {

        });
    }

    useEffect(() => {
        listen<string>('message', (event) => {
            pushMessage(event.payload);
        });

        listen<string>('kicked', (_event) => {
            // kicked, event.payload is reason
        });

        listen('server_result', (event) => {
            const result = event.payload;
            console.log('Finished chatroom execution');
            onConnectionEnded(result);
        });
    }, []);

    return <>
        <MessageList messages={messages}></MessageList>
        <TextInput onEnter={sendMessage} placeholder="Write a message..." clearContentOnEnter={true}></TextInput>
    </>
}

export default ChatView;
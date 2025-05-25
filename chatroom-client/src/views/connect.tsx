import TextInput from "../elements/textinput.tsx";
import {useEffect, useState} from "react";
import { invoke } from '@tauri-apps/api/core'
import {listen} from "@tauri-apps/api/event";

type ExitReason = "Kicked" | "Disconnected" | "LostConnection";

type ServerResult = { Ok: ExitReason } | { Err: string };

type ConnectProps = {
    onConnect: (ip: string) => void;
}

function ConnectView({ onConnect }: ConnectProps) {
    const [status, setStatus] = useState("Idle");

    const [ip, setIp] = useState("");
    const [username, setUsername] = useState("");

    useEffect(() => {
        listen<string>('status', (event) => {
            setStatus(event.payload);
        });

        listen('server_result', (event) => {
            let payload = event.payload as ServerResult;

            if ('Ok' in payload) {
                setStatus('Left chatroom.');
            } else {
                setStatus(`Left chatroom due to error: ${payload.Err}`);
            }
        });
    }, []);

    async function attemptConnect() {
        await invoke('connect', { ip: ip, username: username }).then(() => {
            onConnect(ip);
        }).catch((err) => {
            setStatus(err);
        });
    }

    return <div className="flex flex-col items-center justify-center w-full h-full gap-2">
        <p>Connect to a chatroom server:</p>
        <TextInput placeholder="IP : PORT" onChangeE={(e) => { setIp(e.target.value) }}></TextInput>
        <TextInput placeholder="Username" onChangeE={(e) => { setUsername(e.target.value) }}></TextInput>
        <p>{status}</p>
        <button className="border" onClick={() => {
            attemptConnect();
        }}>Connect</button>
    </div>
}

export default ConnectView
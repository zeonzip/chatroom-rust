import './App.css'
import ChatView from "./views/chat.tsx";
import ConnectView from "./views/connect.tsx";
import * as React from "react";

function App() {
    const [connected, setConnected] = React.useState(false);

    function handleDisconnect(_result: any) {
        setConnected(false);
    }

    return connected ? (
        <ChatView onConnectionEnded={handleDisconnect}></ChatView>
    ) : (
        <ConnectView onConnect={() => {
            setConnected(true);
        }}></ConnectView>
    );
}
export default App

import { useCallback, useEffect } from 'react';
import { Unity, useUnityContext } from 'react-unity-webgl';

const Game = () => {
    const { unityProvider, addEventListener, removeEventListener, sendMessage } = useUnityContext(
        {
            loaderUrl: "/GameBuild.loader.js",
            dataUrl: "/GameBuild.data",
            frameworkUrl: "/GameBuild.framework.js",
            codeUrl: "/GameBuild.wasm",
        }
    );

    const onLeaveMatch = useCallback(() => {
    }, []);
    
    const updateSelectedMovement = useCallback((movement: string) => {
        console.log(movement);
        alert(movement);
    }, []);

    useEffect(() => {
        addEventListener("onLeaveMatch", onLeaveMatch);
        addEventListener("updateSelectedMovement", updateSelectedMovement);
        return () => {
            removeEventListener("onLeaveMatch", onLeaveMatch);
            removeEventListener("updateSelectedMovement", updateSelectedMovement);
        }
    }, [unityProvider, addEventListener, onLeaveMatch]);

    useEffect(() => {
        sendMessage('EventHandler', 'OnStartMatch', "nico,nico2");
    }, [sendMessage]);

    return (
        <div>
            <Unity unityProvider={unityProvider} style={{ height: 600, width: 900 }} />
        </div>
    );
}

export default Game;
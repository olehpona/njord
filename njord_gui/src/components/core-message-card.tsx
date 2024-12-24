import { CoreMessage, CoreMessageKind } from "@/types/api";
import { Badge } from "./ui/badge";


export default function CoreMessageCard(props: {message: CoreMessage}){

    let color;

    switch (props.message.kind) {
        case CoreMessageKind.Error:
            color = "bg-error text-white"
            break
        case CoreMessageKind.Info:
            color = "bg-info"
            break
        case CoreMessageKind.Warning:
            color = "bg-warning"
            break
    }

    return (
        <Badge variant={'outline'} className={"w-full " + color} >{props.message.kind} {props.message.message}</Badge>
    )
}
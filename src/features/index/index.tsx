import DueToday from "./due-today"
import WithDeadline from "./with-deadline"
import Inbox from "./inbox"

function Index() {
    return (
        <div>
            <div className="flex">
                <DueToday />
                <WithDeadline />
            </div>
            <Inbox />
        </div>
    )
}

export default Index
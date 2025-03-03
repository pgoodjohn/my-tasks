import DueToday from "./due-today"
import WithDeadline from "./with-deadline"
import Inbox from "./inbox"
import FavoriteProjects from "./favorite-projects"

function Index() {
    return (
        <div className="space-y-8">
            <div className="flex">
                <DueToday />
                <WithDeadline />
            </div>
            <Inbox />
            <FavoriteProjects />
        </div>
    )
}

export default Index
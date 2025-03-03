import DueToday from "./due-today"
import Inbox from "./inbox"
import FavoriteProjects from "./favorite-projects"

function Index() {
    return (
        <div className="space-y-8">
            <div className="flex">
                <DueToday />
            </div>
            <Inbox />
            <FavoriteProjects />
        </div>
    )
}

export default Index
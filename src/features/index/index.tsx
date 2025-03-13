import DueToday from "./due-today"
import Inbox from "./inbox"
import FavoriteProjects from "./favorite-projects"
import { Separator } from "@/components/ui/separator"

function Index() {
    return (
        <div className="space-y-8">
            <div className="flex">
                <DueToday />
            </div>
            <Separator />
            <p className="text-sm text-muted-foreground">Inbox</p>
            <Inbox />
            <Separator />
            <FavoriteProjects />
        </div>
    )
}

export default Index
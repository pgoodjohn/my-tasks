import DueToday from "./due-today"
import Tasks from "./Tasks"
import ContributionsCalendar from "@/components/contributions-calendar"
import WithDeadline from "./with-deadline"

function Index() {
    return (
        <div>
            <div className="flex">
                <DueToday />
                <WithDeadline />
            </div>
            <Tasks />
            <ContributionsCalendar />
        </div>
    )
}

export default Index
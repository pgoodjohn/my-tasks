import DueToday from "./due-today"
import Tasks from "./Tasks"
import ContributionsCalendar from "@/components/contributions-calendar"

function Index() {
    return (
        <div>
            <DueToday />
            <Tasks />
            <ContributionsCalendar />
        </div>
    )
}

export default Index
import { useTasksDueToday } from "@/hooks/use-tasks-due-today";
import TasksTable from "@/components/tasks-table";

const DueToday = () => {
    return (
        <div className="container pr-2">
            <h1>Due Today</h1>
            <DueTodayTable />
        </div>
    );
}

export default DueToday

const DueTodayTable = () => {
    const { data: tasks, isLoading, isError } = useTasksDueToday();

    if (isLoading) {
        return <div>Loading...</div>
    }

    if (isError) {
        return <div>Error loading tasks due today</div>
    }

    if (!tasks || tasks.length === 0) {
        return <div>No tasks due today</div>
    }

    return (
        <div className="py-2">
            <TasksTable tasks={tasks} hiddenColumns={[]} showHeaders={false} />
        </div>
    )
}
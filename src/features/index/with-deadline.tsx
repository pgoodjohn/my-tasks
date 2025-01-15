import { invoke_tauri_command } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import TasksTable from "@/components/tasks-table";

const WithDeadline = () => {

    return (
        <div className="container pl-2">
            <h1>With Deadline</h1>
            <DueTodayTable />
        </div>
    );
}

export default WithDeadline

const DueTodayTable = () => {
    const overdueTasksQuery = useQuery({
        queryKey: ['tasks', 'deadline'],
        queryFn: () => invoke_tauri_command('load_tasks_with_deadline_command', {}),
    });

    if (overdueTasksQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (overdueTasksQuery.isError) {
        return <div>Error loading tasks due today</div>
    }

    if (overdueTasksQuery.data.length === 0) {
        return <div>No tasks due today</div>
    }

    return (
        <div className="py-2">
            <TasksTable tasks={overdueTasksQuery.data} hiddenColumns={[]} />
        </div>
    )

}
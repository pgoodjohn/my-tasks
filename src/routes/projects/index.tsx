import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import { ColumnDef } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'
import EditProjectDialog from '@/components/EditProjectDialog'

export const Route = createFileRoute('/projects/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <ProjectsOverview />
}

const ProjectsOverview: React.FC = () => {
    return (
        <div>
            <p>Projects Overview</p>
            <ProjectsDetailedList />
        </div>
    )
}

type Project = {
    id: string
    title: string
    description: string | null
}

const projectOverviewColumns: ColumnDef<Project>[] = [
    {
        accessorKey: "title",
        header: "Title",
    },
    {
        id: "actions",
        cell: ({ row }) => {
            const project = row.original

            return <EditProjectDialog project={project} />
        }
    }
]

const ProjectsDetailedList: React.FC = () => {
    const projectsListQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_projects_command', {})
            return data
        }
    })

    if (projectsListQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (projectsListQuery.isError) {
        return <div>Error loading tasks</div>
    }

    if (projectsListQuery.data) {
        console.debug("Loaded Data", projectsListQuery.data)
    }

    return (
        <div>
            {projectsListQuery.data ? <DataTable data={projectsListQuery.data} columns={projectOverviewColumns} /> : <div>No Data</div>}
        </div>
    )
}
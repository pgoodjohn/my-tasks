import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import { ColumnDef } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'
import EditProjectDialog from '@/features/projects/EditProjectDialog'
import { Button } from '@/components/ui/button'

export const Route = createFileRoute('/projects/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <ProjectsOverview />
}

const ProjectsOverview: React.FC = () => {
    return (
        <div>
            <p className='text-xl'>Projects Overview</p>
            <div className='pt-2'>
                <ProjectsDetailedList />
            </div>
        </div>
    )
}

type Project = {
    id: string
    title: string
    emoji: string | null,
    description: string | null
}

const projectOverviewColumns: ColumnDef<Project>[] = [
    {
        id: "id",
        accessorKey: "id",
        header: "",
        cell: ({ row }) => {
            const project = row.original
            return (
                <Link to={`/projects/${project.id}`}>
                    <Button size="sm">→</Button>
                </Link>
            )
        }
    },
    {
        accessorKey: "title",
        header: "Title",
        cell: ({ row }) => {
            const project = row.original
            return (
                <div>{project.emoji ?? ""}{project.title}</div>
            )
        }
    },
    {
        id: "openTasks",
        header: "Open Tasks",
        cell: ({ row }) => {
            const project = row.original

            const openTasksForProjectQuery = useQuery({
                queryKey: ['tasks', 'project', project.id, 'open'],
                queryFn: async () => {
                    return invoke_tauri_command('count_open_tasks_for_project_command', { projectId: project.id })
                }
            })

            if (openTasksForProjectQuery.isLoading) {
                return <div></div>
            }

            if (openTasksForProjectQuery.isError) {
                return <div>Error Counting</div>
            }

            if (openTasksForProjectQuery.data) {
                console.debug("Loaded Data", openTasksForProjectQuery.data)
            }

            return (
                <div>{openTasksForProjectQuery.data}</div>
            )

        }
    },
    {
        id: "actions",
        size: 100,
        cell: ({ row }) => {
            const project = row.original
            return (
                <div className='flex justify-end items-center'>
                    <EditProjectDialog project={project} />
                    <FavoriteProjectButton project={project} />
                    <ArchiveProjectButton project={project} />
                </div>
            )
        }
    },
]

import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useConfiguration } from '@/hooks/use-configuration'

const FavoriteProjectButton: React.FC<{ project: Project }> = ({ project }) => {

    const { data: configurationData } = useConfiguration()

    const queryClient = useQueryClient()

    const favoriteMutation = useMutation({
        mutationFn: async () => {
            return invoke_tauri_command('add_project_to_favourites_command', { projectUuid: project.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            queryClient.invalidateQueries({ queryKey: ['configuration'] })
        }
    })

    return (
        <Button size="sm" disabled={favoriteMutation.isPending || configurationData.favoriteProjectsUuids?.includes(project.id)} onClick={() => {
            favoriteMutation.mutateAsync()
        }}>
            Favorite
        </Button>
    )
}

import { toast } from 'sonner'

const ArchiveProjectButton: React.FC<{ project: Project }> = ({ project }) => {

    const queryClient = useQueryClient()

    const archiveMutation = useMutation({
        mutationFn: async () => {
            return invoke_tauri_command('archive_project_command', { projectId: project.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            queryClient.invalidateQueries({ queryKey: ['configuration'] })
            toast.success(`Project ${project.title} was archived`)
        },
    })

    return (
        <Button size="sm" disabled={archiveMutation.isPending} onClick={() => archiveMutation.mutateAsync()}>Archive</Button>
    )
}

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
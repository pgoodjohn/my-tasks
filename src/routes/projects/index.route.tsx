import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import { ColumnDef } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'
import EditProjectDialog from '@/features/projects/EditProjectDialog'
import { Button } from '@/components/ui/button'
import { toast } from 'sonner'
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { MoreHorizontal } from "lucide-react"
import { Project } from '@/types'


export const Route = createFileRoute('/projects/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <ProjectsOverview />
}

const ProjectsOverview: React.FC = () => {
    return (
        <div>
            <div className='flex'>
                <p className='text-xl'>Projects Overview</p>
                <div className='flex-grow' />
                <CreateProjectDialog />
            </div>
            <div className='pt-2'>
                <ProjectsDetailedList />
            </div>
        </div>
    )
}

const projectOverviewColumns: ColumnDef<Project>[] = [
    {
        id: "actions",
        cell: ({ row }) => {
            const project = row.original
            return (
                <div className='items-center max-w-0.5'>
                    <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                            <Button variant="ghost">
                                <span className="sr-only">Open menu</span>
                                <MoreHorizontal />
                            </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuLabel>Actions</DropdownMenuLabel>
                            <DropdownMenuSeparator />
                            <FavoriteProjectButton project={project} />
                            <ArchiveProjectButton project={project} />
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            )
        }
    },
    {
        accessorKey: "title",
        header: "Title",
        cell: ({ row }) => {
            const project = row.original
            return (
                <ProjectTag project={project} asLink />
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
                <div className='font-medium'>{openTasksForProjectQuery.data}</div>
            )

        }
    },
    {
        id: "edit",
        cell: ({ row }) => {
            const project = row.original
            return (
                <div className='max-w-0.5'>
                    <EditProjectDialog project={project} />
                </div>
            )
        }
    }
]

import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useConfiguration } from '@/hooks/use-configuration'
import ProjectTag from '@/components/project-tag'

const FavoriteProjectButton: React.FC<{ project: Project }> = ({ project }) => {

    const { data: configurationData } = useConfiguration()

    const queryClient = useQueryClient()

    const favoriteMutation = useMutation({
        mutationFn: async () => {
            if (configurationData.favoriteProjectsUuids?.includes(project.id)) {
                return invoke_tauri_command('remove_project_from_favourites_command', { projectUuid: project.id })
            }
            return invoke_tauri_command('add_project_to_favourites_command', { projectUuid: project.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            queryClient.invalidateQueries({ queryKey: ['configuration'] })
        }
    })

    return (
        <DropdownMenuItem disabled={favoriteMutation.isPending} onClick={() => {
            favoriteMutation.mutateAsync()
        }}>
            {configurationData.favoriteProjectsUuids?.includes(project.id) && "Unfavorite"}
            {configurationData.favoriteProjectsUuids?.includes(project.id) === false && "Favorite"}
        </DropdownMenuItem>
    )
}

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
        <DropdownMenuItem disabled={archiveMutation.isPending} onClick={() => archiveMutation.mutateAsync()}>Archive</DropdownMenuItem>
    )
}

import { useState } from 'react'
import { Checkbox } from '@/components/ui/checkbox'
import CreateProjectDialog from '@/features/projects/CreateProjectDialog'

const ProjectsDetailedList: React.FC = () => {

    const [showArchived, setShowArchived] = useState(false)

    const projectsListQuery = useQuery({
        queryKey: ['projects', showArchived],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_projects_command', { showArchivedProjects: showArchived })
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
            <div className="flex space-x-2 pb-4">
                <Checkbox id="show-completed" checked={showArchived} onCheckedChange={() => setShowArchived(!showArchived)} />
                <label
                    htmlFor="show-completed"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                    Show Archived
                </label>
            </div>
            {projectsListQuery.data ? <ProjectDetailsTable projects={projectsListQuery.data} /> : <div>No Projects</div>}
        </div>
    )
}

interface ProjectDetailsTableProps {
    projects: Project[]
}

const ProjectDetailsTable: React.FC<ProjectDetailsTableProps> = ({ projects }) => {
    return (
        <DataTable data={projects} columns={projectOverviewColumns} />
    )
}
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { MoreHorizontal } from "lucide-react"
import { useState } from 'react'
import type { ColumnDef } from '@tanstack/react-table'
import { invoke_tauri_command } from '@/lib/utils'
import { DataTable } from '@/components/data-table'
import { Button } from '@/components/ui/button'
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import type { Project } from '@/types'

import ProjectTag from '@/components/project-tag'

import { Checkbox } from '@/components/ui/checkbox'
import CreateProjectDialog from '@/features/projects/create-project-dialog'


const Index: React.FC = () => {
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

export default Index

const projectOverviewColumns: Array<ColumnDef<Project>> = [
    {
        id: "actions",
        size: 10,
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
                <ProjectTag projectId={project.id} asLink />
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

            return (
                <div className='font-medium'>{openTasksForProjectQuery.data}</div>
            )

        }
    },
]

const FavoriteProjectButton: React.FC<{ project: Project }> = ({ project }) => {

    const queryClient = useQueryClient()

    const favoriteMutation = useMutation({
        mutationFn: async () => {
            if (project.isFavorite) {
                return invoke_tauri_command('remove_favorite_project_command', { projectId: project.id })
            }
            return invoke_tauri_command('add_favorite_project_command', { projectId: project.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['projects'] })
        }
    })

    return (
        <DropdownMenuItem disabled={favoriteMutation.isPending} onClick={() => {
            favoriteMutation.mutateAsync()
        }}>
            {project.isFavorite && "Unfavorite"}
            {project.isFavorite === false && "Favorite"}
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

const ProjectsDetailedList: React.FC = () => {

    const [showArchived, setShowArchived] = useState(false)

    const projectsListQuery = useQuery({
        queryKey: ['projects', showArchived],
        queryFn: async () => {
            const data = await invoke_tauri_command('load_projects_command', { showArchivedProjects: showArchived })
            return data
        }
    })

    if (projectsListQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (projectsListQuery.isError) {
        return <div>Error loading tasks</div>
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
    projects: Array<Project>
}

const ProjectDetailsTable: React.FC<ProjectDetailsTableProps> = ({ projects }) => {
    return (
        <DataTable data={projects} columns={projectOverviewColumns} />
    )
}
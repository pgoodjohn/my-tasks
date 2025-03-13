import { Link, useRouterState } from '@tanstack/react-router'
import type { Project } from "@/types"
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarMenuSub,
    SidebarMenuSubButton,
    SidebarMenuSubItem,
} from "@/components/ui/sidebar"
import { useFavoriteProjects } from "@/hooks/use-favorite-projects"
import { useTasksDueToday } from "@/hooks/use-tasks-due-today"
import { useTasks } from "@/hooks/use-tasks"
import { useInboxTasks } from "@/hooks/use-inbox-tasks"

import { Footer } from './footer'
import { SidebarTaskCountBadge } from './sidebar-task-count-badge'
import { ThemeSwitcher } from './theme-switcher'

export function AppSidebar() {
    const tasks = useTasks(false);
    const tasksDueToday = useTasksDueToday();
    const inboxTasks = useInboxTasks();
    const routerState = useRouterState();
    const currentRoute = routerState.matches[1]?.routeId;
    const currentPath = routerState.location.pathname;

    return (
        <Sidebar>
            <SidebarHeader />
            <SidebarContent className="no-scrollbar">
                <SidebarGroup>
                    <SidebarGroupLabel>My Tasks</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuButton asChild isActive={currentRoute === "/"}>
                                <Link to='/'>
                                    <span className="flex justify-between items-center w-full">
                                        <p>
                                            Home
                                        </p>
                                        <div className="flex gap-1">
                                            {
                                                inboxTasks && inboxTasks.data && inboxTasks.data.length > 0 && (
                                                    <SidebarTaskCountBadge count={inboxTasks.data.length} variant="blue" />
                                                )
                                            }
                                            {
                                                tasksDueToday && tasksDueToday.data && tasksDueToday.data.length > 0 && (
                                                    <SidebarTaskCountBadge count={tasksDueToday.data.length} variant="orange" />
                                                )
                                            }
                                        </div>
                                    </span>
                                </Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild isActive={currentRoute === "/tasks/"}>
                                <Link to='/tasks' className="flex justify-between items-center w-full">
                                    Tasks
                                    {
                                        tasks && tasks.data && (
                                            <SidebarTaskCountBadge count={tasks.data.length} variant="default" />
                                        )
                                    }
                                </Link>
                            </SidebarMenuButton>
                            <SidebarMenuItem>
                                <SidebarMenuButton asChild isActive={currentRoute === "/projects/"}>
                                    <Link to='/projects'>Projects</Link>
                                </SidebarMenuButton>
                                <FavoriteProjects currentPath={currentPath} />
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
                <SidebarGroup />
            </SidebarContent>
            <SidebarFooter>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuButton asChild isActive={currentRoute === "/tasks/completed"}>
                                <Link to='/tasks/completed'>Completed Tasks</Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild isActive={currentRoute === "/statistics"}>
                                <Link to='/statistics'>Statistics</Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild isActive={currentRoute === "/ollama"}>
                                <Link to='/ollama'>AI Assistant</Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild isActive={currentRoute === "/settings"}>
                                <Link to='/settings'>Settings</Link>
                            </SidebarMenuButton>
                        </SidebarMenu>
                        <SidebarMenu>
                            <ThemeSwitcher>
                                <SidebarMenuButton>
                                    Theme
                                </SidebarMenuButton>
                            </ThemeSwitcher>
                        </SidebarMenu>
                        <SidebarMenu>
                            <SidebarMenuButton disabled>
                                <Footer />
                            </SidebarMenuButton>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarFooter>
        </Sidebar >
    )
}

interface FavoriteProjectsProps {
    currentPath: string;
}

const FavoriteProjects: React.FC<FavoriteProjectsProps> = ({ currentPath }) => {
    const { data, isLoading, error } = useFavoriteProjects();

    if (isLoading || error) {
        return <></>
    }

    return data?.map((project: Project) => {
        const isActive = currentPath === `/projects/${project.id}`;
        return (
            <SidebarMenuSub key={project.id}>
                <SidebarMenuSubItem>
                    <SidebarMenuSubButton asChild isActive={isActive}>
                        <Link to="/projects/$projectId" params={{ projectId: `${project.id}` } as any}>
                            {project.emoji} {project.title}
                        </Link>
                    </SidebarMenuSubButton>
                </SidebarMenuSubItem>
            </SidebarMenuSub >
        )
    })
}

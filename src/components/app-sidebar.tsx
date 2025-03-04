import { Link, useRouterState } from '@tanstack/react-router'
import { Badge } from "./ui/badge"
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
import { useConfiguration } from "@/hooks/use-configuration"
import { useFavoriteProjects } from "@/hooks/use-favorite-projects"
import { useTasksDueToday } from "@/hooks/use-tasks-due-today"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useTheme } from "@/components/theme-provider"
import { useTasks } from "@/hooks/use-tasks"

export function AppSidebar() {
    const tasks = useTasks(false);
    const tasksDueToday = useTasksDueToday();
    const routerState = useRouterState();
    const currentRoute = routerState.matches[1]?.routeId;
    const currentPath = routerState.location.pathname;

    return (
        <Sidebar>
            <SidebarHeader />
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>My Tasks</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuButton asChild isActive={currentRoute === "/"}>
                                <Link to='/' className="flex justify-between items-center w-full">
                                    Home
                                    {
                                        tasksDueToday && tasksDueToday.data && tasksDueToday.data.length > 0 && (
                                            <Badge variant="small-orange">{tasksDueToday.data.length}</Badge>
                                        )
                                    }
                                </Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild isActive={currentRoute === "/tasks/"}>
                                <Link to='/tasks' className="flex justify-between items-center w-full">
                                    Tasks
                                    {
                                        tasks && tasks.data && (
                                            <Badge variant="small">{tasks.data.length}</Badge>
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
                            {/* <SidebarMenuItem>
                                <SidebarMenuButton>
                                    <ContributionsCalendar variant="monthly" />
                                </SidebarMenuButton>
                            </SidebarMenuItem> */}
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
        </Sidebar>
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
                        <Link to="/projects/$projectId" params={{ projectId: project.id }}>
                            {project.emoji} {project.title}
                        </Link>
                    </SidebarMenuSubButton>
                </SidebarMenuSubItem>
            </SidebarMenuSub >
        )
    })
}

const Footer: React.FC = () => {
    const { data, isLoading, error } = useConfiguration();

    if (isLoading) {
        return <></>
    }

    if (error) {
        return <div>Error loading configuration: {error.message}</div>
    }

    return (
        <div className="flex text-center">
            {
                data.developmentMode && (
                    <p className="text-orange-500">👷 v{data.version} 🚧 </p>
                )
            }
            {
                data.developmentMode == false && (
                    <p>
                        v{data.version}
                    </p>
                )
            }
        </div>
    )
}

interface ModeToggleProps {
    children: React.ReactNode
}

const ThemeSwitcher: React.FC<ModeToggleProps> = ({ children }) => {
    const { setTheme } = useTheme()

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                {children}
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
                <DropdownMenuItem onClick={() => setTheme("light")}>
                    Light
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("dark")}>
                    Dark
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("system")}>
                    System
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    )
}
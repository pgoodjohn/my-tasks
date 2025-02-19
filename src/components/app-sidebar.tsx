import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    SidebarGroupLabel,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuItem,
    SidebarMenuButton,
    SidebarMenuSub,
    SidebarMenuSubItem,
    SidebarMenuSubButton,
} from "@/components/ui/sidebar"
import { Link } from '@tanstack/react-router'
import { useConfiguration } from "@/hooks/use-configuration"
import { useFavoriteProjects } from "@/hooks/use-favorite-projects"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useTheme } from "@/components/theme-provider"
import { Project } from "@/types"


export function AppSidebar() {

    return (
        <Sidebar>
            <SidebarHeader />
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>My Tasks</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuButton asChild>
                                <Link to='/'>Home</Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild>
                                <Link to='/tasks'>Tasks</Link>
                            </SidebarMenuButton>
                            <SidebarMenuItem>
                                <SidebarMenuButton asChild>
                                    <Link to='/projects'>Projects</Link>
                                </SidebarMenuButton>
                                <FavoriteProjects />
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
                            <SidebarMenuButton asChild>
                                <Link to='/statistics'>Statistics</Link>
                            </SidebarMenuButton>
                            <SidebarMenuButton asChild>
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

const FavoriteProjects: React.FC = () => {

    const { data, isLoading, error } = useFavoriteProjects();

    if (isLoading || error) {
        return <></>
    }

    return data?.map((project: Project) => {
        return (
            <SidebarMenuSub key={project.id}>
                <SidebarMenuSubItem>
                    <SidebarMenuSubButton asChild>
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
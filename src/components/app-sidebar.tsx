import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    SidebarGroupLabel,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuAction,
} from "@/components/ui/sidebar"
import { Link } from '@tanstack/react-router'
import { useConfiguration } from "@/hooks/use-configuration"
import { ModeToggle } from "./mode-toggle"

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
                                <Link to='/projects'>Projects</Link>
                            </SidebarMenuButton>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
                <SidebarGroup />
            </SidebarContent>
            <SidebarFooter>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuButton asChild>
                                <Link to='/settings' disabled>Settings</Link>
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

const Footer: React.FC = () => {
    const { data, isLoading, error } = useConfiguration();

    if (isLoading) {
        return <></>
    }

    if (error) {
        return <div>Error loading configuration: {error.message}</div>
    }

    console.debug("Loaded Configuration", data)

    return (
        <div className="flex">
            {
                data.developmentMode && (
                    <p className="text-red-500">🚨 {data.version} - Development 🚨</p>
                )
            }
            {
                data.developmentMode == false && (
                    <p>
                        {data.version}
                    </p>
                )
            }
        </div>
    )
}


import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useTheme } from "@/components/theme-provider"

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

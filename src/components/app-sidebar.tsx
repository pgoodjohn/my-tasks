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
    SidebarMenuItem,
    SidebarMenuBadge
} from "@/components/ui/sidebar"
import { Link } from '@tanstack/react-router'
import { useConfiguration } from "@/hooks/use-configuration"

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

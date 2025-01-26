import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { useProjects } from "@/hooks/use-projects";
import { Link } from '@tanstack/react-router'
import { useRouterState } from "@tanstack/react-router"

export default function AppBreadcrumb() {

    return (
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItems />
            </BreadcrumbList>
        </Breadcrumb>

    )
}

function BreadcrumbItems() {
    const routerState = useRouterState();
    const match = routerState.matches[1];

    if (match.routeId == "/") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Home
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    return (
        <>
            <BreadcrumbItem>
                <BreadcrumbLink>
                    <Link to='/'>Home</Link>
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItemFromRouterMatch match={match} />
        </>
    )
}

function BreadcrumbItemFromRouterMatch({ match }: any) {

    if (match.routeId == "/tasks/") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Tasks
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    if (match.routeId.startsWith("/projects")) {
        const projectId = match.id.match(/^\/projects\/([^/]+)/)?.[1];
        return (
            <ProjectBreadcrumb projectId={projectId} />
        )
    }

    if (match.routeId == "/statistics") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Statistics
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    if (match.routeId == "/settings") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Settings
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    return (
        <>
        </>
    )
}

function ProjectBreadcrumb({ projectId }: { projectId: string | undefined }) {

    const projects = useProjects();

    if (!projectId) {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Projects
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    const project = projects.data?.find((item) => item.id === projectId);

    return (
        <>
            <BreadcrumbItem>
                <BreadcrumbLink asChild>
                    <Link to="/projects">Projects</Link>
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <BreadcrumbPage>
                    {project?.title}
                </BreadcrumbPage>
            </BreadcrumbItem >
        </>
    )
}
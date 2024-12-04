import { Project } from '@/types'
import { Link } from '@tanstack/react-router'
import { Badge } from './ui/badge';

interface ProjecTagProps {
    project: Project;
    asLink?: boolean;
}

const ProjectTag: React.FC<ProjecTagProps> = ({ project, asLink = false }) => {

    if (asLink) {
        return (
            <Link to={`/projects/${project.id}`}>
                <Badge>
                    <span>{project.emoji}</span>
                    <span>{project.title}</span>
                </Badge>
            </Link>
        );
    }

    return (
        <Badge>
            <span>{project.emoji}</span>
            <span>{project.title}</span>
        </Badge>
    )
}

export default ProjectTag
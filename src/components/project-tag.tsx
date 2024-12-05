import { Project } from '@/types'
import { Link } from '@tanstack/react-router'
import { Badge } from './ui/badge';
import { cn } from '@/lib/utils';

interface ProjecTagProps {
    project: Project;
    asLink?: boolean;
}

const ProjectTag: React.FC<ProjecTagProps> = ({ project, asLink = false }) => {
    // Determine badge classes based on project properties
    const badgeClasses = cn({
        'bg-blue-400 hover:bg-blue-500 dark:hover:bg-blue-300': project.color === 'blue',
        'bg-red-400 hover:bg-red-500 dark:hover:bg-red-300': project.color === 'red',
    });

    if (asLink) {
        return (
            <Link to={`/projects/${project.id}`}>
                <Badge className={badgeClasses}>
                    <span>{project.emoji}</span>
                    <span>{project.title}</span>
                </Badge>
            </Link>
        );
    }

    return (
        <Badge className={badgeClasses}>
            <span>{project.emoji}</span>
            <span>{project.title}</span>
        </Badge>
    )
}

export default ProjectTag
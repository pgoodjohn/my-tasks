import { Project } from '@/types'
import { Link } from '@tanstack/react-router'
import { Badge } from './ui/badge';
import { cn } from '@/lib/utils';

interface ProjecTagProps {
    project: Project;
    asLink?: boolean;
}

const ProjectTag: React.FC<ProjecTagProps> = ({ project, asLink = false }) => {
    const badgeClasses = cn({
        'bg-red-400 hover:bg-red-500 dark:hover:bg-red-300': project.color === 'red',
        'bg-orange-400 hover:bg-orange-500 dark:hover:bg-orange-300': project.color === 'orange',
        'bg-amber-400 hover:bg-amber-500 dark:hover:bg-amber-300': project.color === 'amber',
        'bg-blue-400 hover:bg-blue-500 dark:hover:bg-blue-300': project.color === 'blue',
        'bg-lime-400 hover:bg-lime-500 dark:hover:bg-lime-300': project.color === 'lime',
        'bg-green-400 hover:bg-green-500 dark:hover:bg-green-300': project.color === 'green',
        'bg-teal-400 hover:bg-teal-500 dark:hover:bg-teal-300': project.color === 'teal',
        'bg-cyan-400 hover:bg-cyan-500 dark:hover:bg-cyan-300': project.color === 'cyan',
        'bg-sky-400 hover:bg-sky-500 dark:hover:bg-sky-300': project.color === 'sky',
        'bg-indigo-400 hover:bg-indigo-500 dark:hover:bg-indigo-300': project.color === 'indigo',
        'bg-violet-400 hover:bg-violet-500 dark:hover:bg-violet-300': project.color === 'violet',
        'bg-purple-400 hover:bg-purple-500 dark:hover:bg-purple-300': project.color === 'purple',
        'bg-pink-400 hover:bg-pink-500 dark:hover:bg-pink-300': project.color === 'pink',
        'bg-rose-400 hover:bg-rose-500 dark:hover:bg-rose-300': project.color === 'rose',
        '': !project.color
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
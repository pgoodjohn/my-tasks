import { Project } from '@/types'
import { Link } from '@tanstack/react-router'

interface ProjecTagProps {
    project: Project;
    asLink?: boolean;
}

const ProjectTag: React.FC<ProjecTagProps> = ({ project, asLink = false }) => {

    if (!asLink) {
        return (
            <div className='border rounded-lg px-2 py-1 inline-block'>
                <span className='text-sm'>{project.emoji}</span>
                <span className='text-sm'>{project.title}</span>
            </div>
        )
    }

    return (
        <Link to={`/projects/${project.id}`} className='border rounded-lg px-2 py-1 inline-block dark:hover:bg-gray-900 hover:bg-gray-200'>
            <span className='text-sm'>{project.emoji}</span>
            <span className='text-sm'>{project.title}</span>
        </Link>
    )
}

export default ProjectTag
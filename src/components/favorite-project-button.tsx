import { useMutation, useQueryClient } from '@tanstack/react-query'
import { Star } from 'lucide-react'
import { Button } from './ui/button'
import type { Project } from '@/types'
import { invoke_tauri_command } from '@/lib/utils'

interface FavoriteProjectButtonProps {
    project: Project
    variant?: 'ghost' | 'default'
    size?: 'default' | 'sm' | 'lg' | 'icon'
}

export function FavoriteProjectButton({ project, variant = 'ghost', size = 'icon' }: FavoriteProjectButtonProps) {
    const queryClient = useQueryClient()

    const favoriteMutation = useMutation({
        mutationFn: async () => {
            if (project.isFavorite) {
                return invoke_tauri_command('remove_favorite_project_command', { projectId: project.id })
            }
            return invoke_tauri_command('add_favorite_project_command', { projectId: project.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            queryClient.invalidateQueries({ queryKey: ['projects', 'favorites'] })
            queryClient.invalidateQueries({ queryKey: ['tasks', 'projects', project.id] })
        }
    })

    return (
        <Button
            variant={variant}
            size={size}
            disabled={favoriteMutation.isPending}
            onClick={() => favoriteMutation.mutateAsync()}
            className={project.isFavorite ? 'text-yellow-500 hover:text-yellow-600' : 'text-muted-foreground hover:text-yellow-500'}
        >
            <Star className="h-4 w-4" fill={project.isFavorite ? 'currentColor' : 'none'} />
        </Button>
    )
} 
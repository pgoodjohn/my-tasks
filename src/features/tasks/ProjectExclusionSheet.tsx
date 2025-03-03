import React from 'react';
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { useProjects } from "@/hooks/use-projects";
import ProjectTag from "@/components/project-tag";

interface ProjectExclusionSheetProps {
    excludedProjects: Array<string>;
    onExcludedProjectsChange: (projects: Array<string>) => void;
}

export const ProjectExclusionSheet: React.FC<ProjectExclusionSheetProps> = ({ excludedProjects, onExcludedProjectsChange }) => {
    const { data: projects, isLoading } = useProjects();

    if (isLoading || !projects) {
        return null;
    }

    const toggleProject = (projectId: string) => {
        if (excludedProjects.includes(projectId)) {
            onExcludedProjectsChange(excludedProjects.filter(id => id !== projectId));
        } else {
            onExcludedProjectsChange([...excludedProjects, projectId]);
        }
    };

    return (
        <Sheet>
            <SheetTrigger asChild>
                <Button variant="outline" size="sm">
                    {excludedProjects.length > 0 ? `${excludedProjects.length} projects hidden` : "Hide Projects"}
                </Button>
            </SheetTrigger>
            <SheetContent>
                <SheetHeader>
                    <SheetTitle>Hide Projects</SheetTitle>
                </SheetHeader>
                <div className="mt-4 space-y-4">
                    {projects.map((project) => (
                        <div key={project.id} className="flex items-center space-x-2">
                            <Checkbox
                                id={`project-${project.id}`}
                                checked={excludedProjects.includes(project.id)}
                                onCheckedChange={() => toggleProject(project.id)}
                            />
                            <label
                                htmlFor={`project-${project.id}`}
                                className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                            >
                                <ProjectTag projectId={project.id} />
                            </label>
                        </div>
                    ))}
                    {excludedProjects.length > 0 && (
                        <Button
                            variant="outline"
                            className="w-full justify-start"
                            onClick={() => onExcludedProjectsChange([])}
                        >
                            Show All Projects
                        </Button>
                    )}
                </div>
            </SheetContent>
        </Sheet>
    );
}; 
"use client"

import * as React from "react"
import { Check, ChevronsUpDown } from "lucide-react"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "@/components/ui/command"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import ProjectTag from "@/components/project-tag"
import { Project } from "@/types"

interface ComboBoxProps {
    values: Project[];
    selectedValue: string | undefined,
    onChange: any;
}

export function Combobox({ values, selectedValue, onChange }: ComboBoxProps) {
    const [open, setOpen] = React.useState(false)
    const selectedProject = values.find((item) => item.title === selectedValue)

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[200px] justify-between"
                >
                    {selectedValue
                        ? <SelectedProjectProps project={selectedProject} projectList={values} />
                        : "No Project"}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[200px] p-0">
                <Command>
                    <CommandInput placeholder="(🚫) Search projects..." />
                    <CommandList>
                        <CommandEmpty>No projects.</CommandEmpty>
                        <CommandGroup>
                            {values.map((item) => (
                                <CommandItem
                                    key={item.id}
                                    value={item.title}
                                    onSelect={(currentValue) => {
                                        onChange(currentValue === selectedValue ? "" : currentValue)
                                        setOpen(false)
                                    }}
                                >
                                    {selectedValue &&
                                        <Check
                                            className={cn(
                                                "mr-2 h-4 w-4",
                                                selectedValue === item.title ? "opacity-100" : "opacity-0"
                                            )}
                                        />
                                    }
                                    <ProjectTag project={item} />
                                </CommandItem>
                            ))}
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    )
}

interface SelectedProjectProps {
    project: Project | undefined;
    projectList: Project[];
}

const SelectedProjectProps: React.FC<SelectedProjectProps> = ({ project, projectList }) => {
    if (!project) {
        return <></>
    }

    if (projectList.find((item) => item.title === project.title)) {
        return <ProjectTag project={projectList.find((item) => item.id === project.id)!} />
    }

    return <> </>

}
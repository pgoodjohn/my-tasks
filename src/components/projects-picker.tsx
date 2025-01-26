import { useState } from "react"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "@/components/ui/command"
import { Button } from "./ui/button"
import { useProjects } from "@/hooks/use-projects"
import { Check, ChevronsUpDown } from "lucide-react"
import ProjectTag from "./project-tag"

import { cn } from "@/lib/utils"

interface ProjectsPickerProps {
    modal: boolean,
    selectedValue: string | undefined,
    onChange: any
}

export function ProjectsPicker({ modal, selectedValue, onChange }: ProjectsPickerProps) {
    const [open, setOpen] = useState(false)

    return (
        <Popover open={open} modal={modal} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[400px] justify-between"
                >
                    {selectedValue ? <ProjectTag projectId={selectedValue ?? ""} /> : "(🚫) No Project"}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] h-[200px] p-0">
                <ProjectsPickerCommand setOpen={setOpen} selectedValue={selectedValue} onChange={onChange} />
            </PopoverContent>
        </Popover >
    )
}

interface ProjectsPickerCommandProps {
    setOpen: any
    onChange: any
    selectedValue?: string
}

function ProjectsPickerCommand({ setOpen, onChange, selectedValue }: ProjectsPickerCommandProps) {
    const [selectedTitle, setSelectedTitle] = useState<string | undefined>(undefined);
    const projects = useProjects();

    return (
        <Command>
            <CommandInput placeholder="(🚫) Search projects..." />
            <CommandList>
                <CommandEmpty>No projects.</CommandEmpty>
                <CommandGroup>
                    {projects.data?.map((item) => (
                        <CommandItem
                            key={item.id}
                            value={item.title} // Change this to item.id instead of item.title
                            onSelect={(currentValue) => {

                                let selectedValueId = projects.data?.find((item) => item.title === currentValue)?.id;

                                onChange(selectedValueId === selectedValue ? "" : selectedValueId) // Change this to selectedValueId instead of currentValue
                                setSelectedTitle(item.title); // Update selectedTitle with item.title
                                setOpen(false);
                            }}
                        >
                            {selectedValue &&
                                <Check
                                    className={cn(
                                        "mr-2 h-4 w-4",
                                        selectedTitle === item.title ? "opacity-100" : "opacity-0" // Use selectedTitle for comparison
                                    )}
                                />
                            }
                            <ProjectTag projectId={item.id} />
                        </CommandItem>
                    ))}
                </CommandGroup>
            </CommandList>
        </Command>
    )
}
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

const colors = [
    { value: "red", label: "Red" },
    { value: "orange", label: "Orange" },
    { value: "amber", label: "Amber" },
    { value: "blue", label: "Blue" },
    { value: "lime", label: "Lime" },
    { value: "green", label: "Green" },
    { value: "teal", label: "Teal" },
    { value: "cyan", label: "Cyan" },
    { value: "sky", label: "Sky" },
    { value: "indigo", label: "Indigo" },
    { value: "violet", label: "Violet" },
    { value: "purple", label: "Purple" },
    { value: "pink", label: "Pink" },
    { value: "rose", label: "Rose" },
]

interface ProjectColorCombooxProps {
    selectedValue: any;
    onChange: any;
}
const ProjectColorCombobox: React.FC<ProjectColorCombooxProps> = ({ selectedValue, onChange }) => {
    const [open, setOpen] = React.useState(false)

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
                        ? colors.find((color) => color.value === selectedValue)?.label
                        : "Select color..."}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[200px] p-0">
                <Command>
                    <CommandInput placeholder="Search color..." />
                    <CommandList>
                        <CommandEmpty>No color found.</CommandEmpty>
                        <CommandGroup>
                            {colors.map((color) => (
                                <CommandItem
                                    key={color.value}
                                    value={color.value}
                                    onSelect={(currentValue) => {
                                        onChange(currentValue === selectedValue ? "" : currentValue)
                                        setOpen(false)
                                    }}
                                >
                                    <Check
                                        className={cn(
                                            "mr-2 h-4 w-4",
                                            selectedValue === color.value ? "opacity-100" : "opacity-0"
                                        )}
                                    />
                                    {color.label}
                                </CommandItem>
                            ))}
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    )
}

export default ProjectColorCombobox
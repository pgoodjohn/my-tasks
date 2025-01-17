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
import { ScrollArea } from "@/components/ui/scroll-area"

const colors = [
    { value: "red", label: "Red" },
    { value: "orange", label: "Orange" },
    { value: "amber", label: "Amber" },
    { value: "lime", label: "Lime" },
    { value: "green", label: "Green" },
    { value: "teal", label: "Teal" },
    { value: "cyan", label: "Cyan" },
    { value: "sky", label: "Sky" },
    { value: "blue", label: "Blue" },
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
        <Popover open={open} modal={true} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="ghost"
                    role="combobox"
                    aria-expanded={open}
                >
                    <ColorSquare color={selectedValue} />
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[200px] p-0">
                <ScrollArea>
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

                                        <ColorSquare color={color.value} /> {color.label}
                                    </CommandItem>
                                ))}
                            </CommandGroup>
                        </CommandList>
                    </Command>
                </ScrollArea>
            </PopoverContent>
        </Popover>
    )
}
interface ColorSquareProps {
    color: string;
}

const ColorSquare: React.FC<ColorSquareProps> = ({ color }) => {

    const badgeClasses = cn({
        'bg-red-400 hover:bg-red-500 dark:hover:bg-red-300': color === 'red',
        'bg-orange-400 hover:bg-orange-500 dark:hover:bg-orange-300': color === 'orange',
        'bg-amber-400 hover:bg-amber-500 dark:hover:bg-amber-300': color === 'amber',
        'bg-lime-400 hover:bg-lime-500 dark:hover:bg-lime-300': color === 'lime',
        'bg-green-400 hover:bg-green-500 dark:hover:bg-green-300': color === 'green',
        'bg-blue-400 hover:bg-blue-500 dark:hover:bg-blue-300': color === 'blue',
        'bg-teal-400 hover:bg-teal-500 dark:hover:bg-teal-300': color === 'teal',
        'bg-cyan-400 hover:bg-cyan-500 dark:hover:bg-cyan-300': color === 'cyan',
        'bg-sky-400 hover:bg-sky-500 dark:hover:bg-sky-300': color === 'sky',
        'bg-indigo-400 hover:bg-indigo-500 dark:hover:bg-indigo-300': color === 'indigo',
        'bg-violet-400 hover:bg-violet-500 dark:hover:bg-violet-300': color === 'violet',
        'bg-purple-400 hover:bg-purple-500 dark:hover:bg-purple-300': color === 'purple',
        'bg-pink-400 hover:bg-pink-500 dark:hover:bg-pink-300': color === 'pink',
        'bg-rose-400 hover:bg-rose-500 dark:hover:bg-rose-300': color === 'rose',
        '': !color
    });

    return (
        <div className={`w-4 h-4 border-2 border-rounded ${badgeClasses}`} aria-label={`Color square with ${color} color`}></div>
    );
};

export default ProjectColorCombobox
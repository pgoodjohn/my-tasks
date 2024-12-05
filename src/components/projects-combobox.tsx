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

interface ComboBoxProps {
    values: any[];
    selectedValue: any;
    onChange: any;
}

export function Combobox({ values, selectedValue, onChange }: ComboBoxProps) {
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
                        ? <ProjectTag project={values.find((item) => item.id === selectedValue)} />
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
                                    value={item.id}
                                    onSelect={(currentValue) => {
                                        onChange(currentValue === selectedValue ? "" : currentValue)
                                        setOpen(false)
                                    }}
                                >
                                    <Check
                                        className={cn(
                                            "mr-2 h-4 w-4",
                                            selectedValue === item.id ? "opacity-100" : "opacity-0"
                                        )}
                                    />
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

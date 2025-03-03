import EmojiPicker from 'emoji-picker-react';
import { useState } from 'react';
import { Button } from './ui/button';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"

interface EmojiPickerFormItemProps {
  value: string | undefined;
  onSelect: (emoji: string) => void;
}

export function EmojiPickerFormItem({ value, onSelect }: EmojiPickerFormItemProps) {
  const [open, setOpen] = useState(false)
  return (
    <>
      <Popover open={open} modal={true} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button variant="outline">
            {value ?? "-"}
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-auto p-0">
          <EmojiPicker
            skinTonesDisabled={true}
            onEmojiClick={
              (e) => {
                onSelect(e.emoji)
                setOpen(false)
              }
            }
          />
        </PopoverContent>
      </Popover>
    </>
  );
}
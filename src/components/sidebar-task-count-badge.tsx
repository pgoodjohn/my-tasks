import { Badge } from "./ui/badge";
import { cn } from "@/lib/utils";

type SidebarTaskCountBadgeVariant = "default" | "orange" | "red" | "blue";

export function SidebarTaskCountBadge({
    count,
    variant = "default"
}: {
    count: number;
    variant?: SidebarTaskCountBadgeVariant;
}) {
    const variantClasses = {
        default: "bg-primary text-primary-foreground hover:bg-primary/80",
        orange: "bg-orange-500 text-white hover:bg-orange-600",
        red: "bg-destructive text-destructive-foreground hover:bg-destructive/80",
        blue: "bg-blue-500 text-white hover:bg-blue-600"
    };

    return (
        <Badge
            variant="default"
            className={cn(
                "px-1 text-xs border-transparent shadow",
                variantClasses[variant]
            )}
        >
            {count}
        </Badge>
    )
}
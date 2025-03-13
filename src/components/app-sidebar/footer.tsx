
import { useConfiguration } from "@/hooks/use-configuration"

export const Footer: React.FC = () => {
    const { data, isLoading, error } = useConfiguration();

    if (isLoading) {
        return <></>
    }

    if (error) {
        return <div>Error loading configuration: {error.message}</div>
    }

    return (
        <div className="flex text-center">
            {
                data.developmentMode && (
                    <p className="text-orange-500">👷 v{data.version} 🚧 </p>
                )
            }
            {
                data.developmentMode == false && (
                    <p>
                        v{data.version}
                    </p>
                )
            }
        </div>
    )
}

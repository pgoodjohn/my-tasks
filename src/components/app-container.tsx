import React from 'react'

interface AppContainerProps {
    children: React.ReactNode
}

const AppContainer: React.FC<AppContainerProps> = ({ children }) => {
    return (
        <div className="w-full max-h-screen p-4">
            {children}
        </div>
    )
}

export default AppContainer
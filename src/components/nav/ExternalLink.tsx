import { ArrowTopRightOnSquareIcon } from "@heroicons/react/20/solid";
import { PropsWithChildren } from "react";

interface ExternalLinkProps {
	href: string;
}

export default function ExternalLink({
	href,
	children,
}: PropsWithChildren<ExternalLinkProps>) {
	return (
		<a
			href={href}
			className="flex items-center text-sm gap-0.5 opacity-70 hover:opacity-100 transition-opacity"
			rel="noreferrer"
			target="_blank"
		>
			{children}
			<ArrowTopRightOnSquareIcon className="w-3" />
		</a>
	);
}

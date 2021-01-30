import React from 'react';
import { Link } from "@reach/router";
import Topbar from './common/Topbar';

function BrowseSources() {
    const [sourceList, setSourceList] = React.useState([]);

    React.useEffect(() => {
        fetch(`/api/source?installed=1`)
            .then((response) => response.json())
            .then((data) => {
                setSourceList(data);
            }).catch((e) => {
                console.log(e);
            });
    }, [])

    return (
        <div className={"lg:pl-48"}>
            <Topbar>
                <button>Filter</button>
                <span className={"text-gray-300"}>Browse</span>
                <span></span>
            </Topbar>
                {sourceList.map((s, index) => (
                    <div key={index} className={"bg-white dark:bg-gray-700 rounded mx-2 p-2 shadow"}>
                        <Link className={"flex justify-between"} to={`/browse/${s.Name}`}>
                            <div className={"inline-flex"}>
                                <img className={"w-10 h-10 mr-2"} src={s.Icon} alt={s.Name}></img>
                                <div>
                                    <div className={"text-gray-900 dark:text-gray-50 text-left"}>{s.Name}</div>
                                    <div className={"text-gray-800 dark:text-gray-200 text-sm text-left"}>{s.Version}</div>
                                </div>
                            </div>
                        </Link>
                    </div>
                ))}
        </div>
    )
}

export default BrowseSources;
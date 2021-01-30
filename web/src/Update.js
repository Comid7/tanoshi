import React from 'react';
import { Link } from '@reach/router';

function Update() {
    const [page, setPage] = React.useState(1);
    const [update, setUpdate] = React.useState([]);
    const [disableLoadMore, setDisableLoadMore] = React.useState(false);

    React.useEffect(() => {
        fetch(`/api/update?page=${page}&limit=10`)
            .then((response) => {
                if (response.status === 204) {
                    setDisableLoadMore(true);
                    return [];
                }
                return response.json();
            })
            .then((data) => {
                setUpdate(u => [...u, ...data]);
            }).catch((e) => {
                console.log(e);
            });
    }, [page]);

    const calculate_days = (at) => {
        let today = Date.now();
        let read = new Date(at);
        let diff = Math.abs(today - read);
        let days = Math.ceil(diff / (1000 * 60 * 60 * 24));

        if (days === 0) {
            return "Today";
        } else if (days === 1) {
            return "Yesterday";
        } else if (days > 1 && days <= 7) {
            return days + " Days Ago";
        } else if (days > 7 && days < 31) {
            return Math.ceil(days / 7) + " Weeks Ago";
        } else {
            return Math.ceil(days / 30) + " Months Ago";
        }
    }

    return (
        <div className={"divide-y divide-gray-100 dark:divide-gray-900 rounded"}>
            {update && update.map((u, i) => (
                <Link key={i} className={"h-auto p-2 flex bg-white dark:bg-gray-800 shadow"} to={`/chapter/${u.ChapterID}`}>
                    <img className={"h-24 rounded object-cover"} alt={u.MangaTitle} src={`/api/proxy?url=${u.CoverURL}`}></img>
                    <div className={"flex flex-col ml-2"}>
                        <h1 className={"text-gray-900 dark:text-gray-50 text-left break-words"}>{u.MangaTitle}</h1>
                        <h2 className={"text-gray-900 dark:text-gray-50 text-left break-words"}>{`${u.ChapterTitle} - ${u.ChapterNumber}`}</h2>
                        <h2 className={"text-gray-900 dark:text-gray-50 text-left break-words"}>{calculate_days(u.UploadedAt)}</h2>
                    </div>
                </Link>
            ))}
            <button disabled={disableLoadMore} className={"w-full p-2 flex h-10 bg-white dark:bg-gray-800 shadow text-accent hover:bg-gray-300 dark:hover:bg-gray-700 justify-center"} onClick={(e) => setPage(page + 1)}>
                {disableLoadMore ? "No More" : "Load More"}
            </button>
        </div>
    )
}
export default Update;
(ns file-explorer
  (:require
   [clojure.core.match :refer [match]]
   [cheshire.core :as json]
   [babashka.fs :as fs]))

;; Functions -------------------------------------------------------------------

(defn list-dir-data-edn [path]
  (let [entries (-> (fs/expand-home path)
                    (fs/list-dir))
        {:keys [dirs files]}
        (reduce (fn [acc cur]
                  (let [dir? (fs/directory? cur)
                        value (fs/file-name cur)
                        item (if dir?
                               {:value value
                                :kind {:SyncShellCommand {:command (str "bb file_explorer " cur)}}
                                :action "Next"}
                               {:value value
                                :action "Exit"})]
                    (cond-> acc
                      dir? (update :dirs conj item)
                      :else (update :files conj item))))
                {:dirs []
                 :files []}
                entries)
        items (concat (sort-by :value dirs) (sort-by :value files))]
    {:value (str "List files: " path)
     :kind "Initial"
     :items items}))

;; Commands --------------------------------------------------------------------

(defn list-dir-cmd [path]
  (-> (list-dir-data-edn path)
      (json/generate-string)
      (doto println)))

(comment
  (list-dir-cmd "~")
  nil)

;; Main ------------------------------------------------------------------------

(defn -main [& args]
  (match (vec args)
         [path] (list-dir-cmd path)
         :else (System/exit 1)))

(apply -main *command-line-args*)

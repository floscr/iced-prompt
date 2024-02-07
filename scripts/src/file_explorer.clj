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
                                :shell (str "bb ./scripts/src/file_explorer.clj " cur)
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
     :shell (str "bb ./scripts/src/file_explorer.clj " path)
     :items items}))

;; Commands --------------------------------------------------------------------

(defn list-dir-cmd [path]
  (-> (list-dir-data-edn path)
      (json/generate-string)
      (doto println)))

(comment
  (list-dir-data-edn "~")
  nil)

;; Main ------------------------------------------------------------------------

(defn -main [& args]
  (match (vec args)
         [path] (list-dir-cmd path)
         :else (System/exit 1)))

(when (= *file* (System/getProperty "babashka.file"))
  (apply -main *command-line-args*))

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
                        title (fs/file-name cur)
                        item (if dir?
                               {:title (str title)
                                :value (str cur)
                                :shell "bb ./scripts/src/file_explorer.clj $__COMMAND_VALUE"
                                :icon "Directory"
                                :action "Next"}
                               {:title (str title)
                                :value (str cur)
                                :icon "File"
                                :action "Exit"})
                        key (if dir? :dirs :files)]
                    (update acc key conj item)))
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
  ;; (Thread/sleep 5000)
  (match (vec args)
         [path] (list-dir-cmd path)
         :else (System/exit 1)))

(when (= *file* (System/getProperty "babashka.file"))
  (apply -main *command-line-args*))

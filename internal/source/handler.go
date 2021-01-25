package source

type Handler struct {
	sm *Manager
}

func NewHandler(sm *Manager) *Handler {
	return &Handler{sm}
}

func (h *Handler) GetSourcesFromRemote() ([]*Source, error) {
	return h.sm.GetSourcesFromRemote()
}

func (h *Handler) InstallSource(name string) error {
	return h.sm.InstallSource(name)
}

func (h *Handler) GetSourceList() ([]*Source, error) {
	return h.sm.List(), nil
}

func (h *Handler) GetSourceDetail(name string) (*Source, error) {
	return h.sm.Get(name), nil
}

func (h *Handler) GetSourceLatestUpdates(name string, page int) ([]*Manga, error) {
	return h.sm.GetLatestUpdates(name, page)
}

func (h *Handler) SearchManga(name string, filter map[string]string) ([]*Manga, error) {
	return h.sm.SearchManga(name, filter)
}

func (h *Handler) GetMangaDetails(id uint, includeChapter bool) (*Manga, error) {
	return h.sm.GetMangaDetails(id, includeChapter)
}

func (h *Handler) GetChapters(mangaID uint) ([]*Chapter, error) {
	return h.sm.GetChapters(mangaID)
}

func (h *Handler) GetChapter(id uint) (*Chapter, error) {
	return h.sm.GetChapter(id)
}

func (h *Handler) Login(name, username, password, twoFactor string, remember bool) error {
	return h.sm.Login(name, username, password, twoFactor, remember)
}

func (h *Handler) SaveFavorite(mangaID uint) error {
	return h.sm.SaveFavorite(mangaID)
}

func (h *Handler) DeleteFavorite(mangaID uint) error {
	return h.sm.DeleteFavorite(mangaID)
}
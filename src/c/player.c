#include <gst/gst.h>
#include <stdio.h>

void * agst_setup(void) {
    gst_init(0, 0);
    return (void*)gst_element_factory_make("playbin", "play");
}

void agst_cleanup(void *gst_player) {
    gst_element_set_state((GstElement *)gst_player, GST_STATE_NULL);
}

void agst_set_file(void *gst_player, char *uri) {
    g_object_set(G_OBJECT((GstElement *)gst_player), "uri", uri, NULL);
}

int agst_play(void *gst_player) {
    return gst_element_set_state((GstElement *)gst_player, GST_STATE_PLAYING);
}

int agst_pause(void *gst_player) {
    return gst_element_set_state((GstElement *)gst_player, GST_STATE_PAUSED);
}

int agst_is_playing(void *gst_player) {
    GstState state;
    gst_element_get_state((GstElement *)gst_player, &state, NULL, 100);
    return state == GST_STATE_PLAYING;
}

int64_t agst_position(void *gst_player) {
    int64_t pos = 0;
    int result = gst_element_query_position((GstElement *)gst_player, GST_FORMAT_TIME, &pos);
    if(result < 0) {
        printf("Failed to get position");
    }
    return GST_TIME_AS_SECONDS(pos);
}

int64_t agst_duration(void *gst_player) {
    int64_t len = 0;
    int result = gst_element_query_duration((GstElement *)gst_player, GST_FORMAT_TIME, &len);
    if(result < 0) {
        printf("Failed to get duration");
    }
    return GST_TIME_AS_SECONDS(len);
}

void agst_seek(void *gst_player, int64_t sec) {
    if (!gst_element_seek_simple((GstElement *)gst_player, GST_FORMAT_TIME, GST_SEEK_FLAG_FLUSH | GST_SEEK_FLAG_ACCURATE, sec * 1000000000)) {
        printf("Seek failed!\n");
    }
}


import { createMutation, createQuery } from "@tanstack/svelte-query";
import axios from "axios";
import { queryClient } from "./client";

export type UrlUpsertDto = {
  url: string;
  destinationUrl: string;
  ttl: Date;
};
export type Url = {
  url: string;
  destinationUrl: string;
  ttl: Date;
  ownedBy: string;
  createdAt: Date;
  updatedAt: Date;
};

export function writeUrl({ url, ...rest }: UrlUpsertDto) {
  return axios.patch(`/api/url/${url}`, rest, { withCredentials: true });
}

export function listUrls() {
  return axios.get<Url[]>("/api/url/list", { withCredentials: true });
}

export function deleteUrl(url: string) {
  return axios.delete(`/api/url/${url}`, { withCredentials: true });
}

export const queryUrls = () =>
  createQuery({
    queryKey: ["urls"],
    queryFn: () => listUrls(),
  });

export const mutateUrl = () =>
  createMutation({
    mutationFn: (data: UrlUpsertDto) => writeUrl(data),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["urls"],
      });
    },
  });

export const mutateDeleteUrl = () =>
  createMutation({
    mutationFn: (data: string) => deleteUrl(data),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["urls"],
      });
    },
  });

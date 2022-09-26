-- MySQL dump 10.16  Distrib 10.1.48-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: manga_server
-- ------------------------------------------------------
-- Server version	10.1.48-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `account`
--

DROP TABLE IF EXISTS `account`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `account` (
  `account_id` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`account_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `author`
--

DROP TABLE IF EXISTS `author`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `author` (
  `author_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`author_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `chapter`
--

DROP TABLE IF EXISTS `chapter`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `chapter` (
  `chapter_id` varchar(255) NOT NULL,
  `chapter_name` varchar(255) NOT NULL,
  `chapter_number` varchar(255) NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `manga_id` varchar(255) DEFAULT NULL,
  `last_watch_time` bigint(20) NOT NULL,
  `sequence_number` int(11) NOT NULL,
  PRIMARY KEY (`chapter_id`),
  KEY `manga_chapter_fk` (`manga_id`),
  CONSTRAINT `manga_chapter_fk` FOREIGN KEY (`manga_id`) REFERENCES `manga` (`manga_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `chapter_page`
--

DROP TABLE IF EXISTS `chapter_page`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `chapter_page` (
  `chapter_page_id` int(11) NOT NULL AUTO_INCREMENT,
  `url` varchar(2048) NOT NULL,
  `page_number` int(11) NOT NULL,
  `chapter_id` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`chapter_page_id`),
  KEY `chapter_chapter_page_fk` (`chapter_id`),
  CONSTRAINT `chapter_chapter_page_fk` FOREIGN KEY (`chapter_id`) REFERENCES `chapter` (`chapter_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=41523920 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `genre`
--

DROP TABLE IF EXISTS `genre`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `genre` (
  `genre_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`genre_id`),
  UNIQUE KEY `genre_name_unique_index` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `linkage_correction`
--

DROP TABLE IF EXISTS `linkage_correction`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `linkage_correction` (
  `linkage_correction_id` int(11) NOT NULL AUTO_INCREMENT,
  `url_from` varchar(2048) NOT NULL,
  `url_to` varchar(2048) NOT NULL,
  `linkage_type` varchar(255) NOT NULL,
  PRIMARY KEY (`linkage_correction_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `manga`
--

DROP TABLE IF EXISTS `manga`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `manga` (
  `manga_id` varchar(255) NOT NULL,
  `linked_id` varchar(255) NOT NULL,
  `is_listed` tinyint(1) NOT NULL,
  `name` varchar(255) NOT NULL,
  `cover_url` varchar(2048) NOT NULL,
  `url` varchar(2048) CHARACTER SET ascii DEFAULT NULL,
  `last_updated` datetime DEFAULT NULL,
  `status` varchar(255) NOT NULL,
  `is_main` tinyint(1) DEFAULT NULL,
  `description` text NOT NULL,
  `source_id` varchar(255) NOT NULL,
  `last_watch_time` bigint(20) DEFAULT NULL,
  `public_id` varchar(255) NOT NULL,
  `is_old` tinyint(1) NOT NULL,
  PRIMARY KEY (`manga_id`),
  UNIQUE KEY `idx_manga_url` (`url`),
  KEY `source_manga_fk` (`source_id`),
  CONSTRAINT `source_manga_fk` FOREIGN KEY (`source_id`) REFERENCES `source` (`source_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `manga_artist`
--

DROP TABLE IF EXISTS `manga_artist`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `manga_artist` (
  `manga_artist_id` int(11) NOT NULL AUTO_INCREMENT,
  `manga_id` varchar(255) NOT NULL,
  `author_id` varchar(255) NOT NULL,
  PRIMARY KEY (`manga_artist_id`),
  KEY `author_manga_artist_fk` (`author_id`),
  CONSTRAINT `author_manga_artist_fk` FOREIGN KEY (`author_id`) REFERENCES `author` (`author_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=565 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `manga_author`
--

DROP TABLE IF EXISTS `manga_author`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `manga_author` (
  `manga_author_id` int(11) NOT NULL AUTO_INCREMENT,
  `author_id` varchar(255) NOT NULL,
  `manga_id` varchar(255) NOT NULL,
  PRIMARY KEY (`manga_author_id`),
  KEY `author_author_genre_fk` (`author_id`),
  KEY `manga_author_genre_fk` (`manga_id`),
  CONSTRAINT `author_author_genre_fk` FOREIGN KEY (`author_id`) REFERENCES `author` (`author_id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT `manga_author_genre_fk` FOREIGN KEY (`manga_id`) REFERENCES `manga` (`manga_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=37830 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `manga_genre`
--

DROP TABLE IF EXISTS `manga_genre`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `manga_genre` (
  `manga_genre_id` int(11) NOT NULL AUTO_INCREMENT,
  `genre_id` varchar(255) NOT NULL,
  `manga_id` varchar(255) NOT NULL,
  PRIMARY KEY (`manga_genre_id`),
  KEY `genre_manga_genre_fk` (`genre_id`),
  KEY `manga_manga_genre_fk` (`manga_id`),
  CONSTRAINT `genre_manga_genre_fk` FOREIGN KEY (`genre_id`) REFERENCES `genre` (`genre_id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT `manga_manga_genre_fk` FOREIGN KEY (`manga_id`) REFERENCES `manga` (`manga_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=220676 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `manga_listing`
--

DROP TABLE IF EXISTS `manga_listing`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `manga_listing` (
  `manga_listing_id` int(11) NOT NULL AUTO_INCREMENT,
  `manga_id` varchar(255) NOT NULL,
  `cover_url` varchar(2048) NOT NULL,
  `name` varchar(255) NOT NULL,
  `genres` varchar(255) NOT NULL,
  `description_small` varchar(255) NOT NULL,
  `public_id` varchar(255) NOT NULL,
  PRIMARY KEY (`manga_listing_id`)
) ENGINE=InnoDB AUTO_INCREMENT=55235 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `problem_child`
--

DROP TABLE IF EXISTS `problem_child`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `problem_child` (
  `problem_child_id` int(11) NOT NULL AUTO_INCREMENT,
  `url` varchar(2048) NOT NULL,
  PRIMARY KEY (`problem_child_id`)
) ENGINE=InnoDB AUTO_INCREMENT=8688 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `source`
--

DROP TABLE IF EXISTS `source`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `source` (
  `source_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `priority` int(11) NOT NULL,
  PRIMARY KEY (`source_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `source_pattern`
--

DROP TABLE IF EXISTS `source_pattern`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `source_pattern` (
  `source_pattern_id` varchar(255) NOT NULL,
  `url` varchar(2048) NOT NULL,
  `source_id` varchar(255) NOT NULL,
  PRIMARY KEY (`source_pattern_id`),
  KEY `source_source_pattern_fk` (`source_id`),
  CONSTRAINT `source_source_pattern_fk` FOREIGN KEY (`source_id`) REFERENCES `source` (`source_id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `title`
--

DROP TABLE IF EXISTS `title`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `title` (
  `title_id` varchar(255) NOT NULL,
  `title` varchar(255) NOT NULL,
  `linked_id` varchar(255) NOT NULL,
  PRIMARY KEY (`title_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2022-09-26 10:15:09
